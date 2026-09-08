"""Contraejemplos del observador, independientes del resultado del compilador."""
import json
import subprocess
import sys
import unittest

from oracle_support import (OracleError, DuplicateJsonMember, ordered_json,
                            assert_json_equal, assert_bytes_equal, assert_success,
                            assert_rust_rejection,
                            cli_payload, run, assert_json_roundtrip)
from run_oracle_sensitivity import assert_sensitivity_rejection


def result(rc=1, out=b"", err=b""):
    return subprocess.CompletedProcess([], rc, out, err)


# N0-02 §2 / IR 0.3 §6.2: cada miembro de K={A,B} aparece una vez,
# sin claves ajenas. Expectativas fijadas por los cuatro testigos fuente,
# independientes del diccionario del observador y de la salida del compilador.
E115_WITNESSES = {
    "output_semantics_vacia": "repetidas=[]; ausentes=[A, B]; ajenas=[]",
    "output_semantics_clave_ausente": "repetidas=[]; ausentes=[B]; ajenas=[]",
    "output_semantics_clave_ajena": "repetidas=[]; ausentes=[]; ajenas=[X]",
    "output_semantics_clave_repetida": "repetidas=[A]; ausentes=[]; ajenas=[]",
}


def e115_result(cause, objects="CellSpec C, OutputSemantics S, Codomain K"):
    message = f"E115 (InvalidOutputSemantics): {objects}: {cause}"
    return result(err=f'SVP no admitido: InvalidProgram("{message}")\n'.encode())


def sensitivity_rejections():
    # Las dos fuentes del banco difieren en la presencia de CellSpec CC.
    return {
        'semantics_duplicate': e115_result(
            'repetidas=[Alpha]; ausentes=[]; ajenas=[]',
            'CellSpec CC, OutputSemantics SS, Codomain KK'),
        'semantics_unbound_duplicate': result(err=(
            b'SVP no admitido: InvalidProgram("E115 (InvalidOutputSemantics): '
            b'OutputSemantics SS: repetidas=[Alpha]")\n')),
    }


class OracleTests(unittest.TestCase):
    def test_sensitivity_uses_its_own_witnesses(self):
        for name, proc in sensitivity_rejections().items():
            with self.subTest(name=name):
                assert_sensitivity_rejection(name, proc)

    def test_sensitivity_rejects_the_other_scope_and_corpus_objects(self):
        for name in sensitivity_rejections():
            others = [proc for other, proc in sensitivity_rejections().items() if name != other]
            others.append(e115_result('repetidas=[A]; ausentes=[]; ajenas=[]'))
            for proc in others:
                with self.subTest(name=name, diagnostic=proc.stderr), self.assertRaises(OracleError):
                    assert_sensitivity_rejection(name, proc)

    def test_e115_accepts_each_normative_witness(self):
        for case, cause in E115_WITNESSES.items():
            with self.subTest(case=case):
                assert_rust_rejection(e115_result(cause), case)

    def test_e115_rejects_each_other_cause(self):
        for case in E115_WITNESSES:
            for other, cause in E115_WITNESSES.items():
                if case != other:
                    with self.subTest(case=case, other=other), self.assertRaises(OracleError):
                        assert_rust_rejection(e115_result(cause), case)

    def test_e115_requires_the_witness_objects(self):
        for case, cause in E115_WITNESSES.items():
            for objects in ["CellSpec Otra, OutputSemantics S, Codomain K",
                            "CellSpec C, OutputSemantics Otra, Codomain K",
                            "CellSpec C, OutputSemantics S, Codomain Otro"]:
                with self.subTest(case=case, objects=objects), self.assertRaises(OracleError):
                    assert_rust_rejection(e115_result(cause, objects), case)

    def test_roundtrip_preserves_local_scope_arrays_and_strings(self):
        raw = '{"b":[{"A":"ñ\\n\\t\\\\"},{"A":"igual"}],"a":[1,1,true,null,{},[]]}'.encode()
        repeated = assert_json_roundtrip(raw)
        self.assertEqual(json.loads(repeated), json.loads(raw))
        self.assertEqual(list(json.loads(repeated)), ['b', 'a'])

    def test_roundtrip_keeps_numeric_tokens_without_machine_or_digit_limit(self):
        raw = b'{"n":' + b'9' * 5000 + b',"z":-0,"e":1.00e-999}'
        self.assertEqual(assert_json_roundtrip(raw), raw)

    def test_roundtrip_rejects_homonyms_after_decoding_escapes(self):
        for raw in [b'{"A":1,"A":1}', b'{"deep":[{"A":1,"\\u0041":2}]}']:
            with self.subTest(raw=raw), self.assertRaises(DuplicateJsonMember):
                assert_json_roundtrip(raw)

    def test_allowed_json_layout(self):
        assert_json_equal(b'{"x": [1, "a"]}', b'{ "x" : [1,"\\u0061"] }\n')

    def test_member_order_is_visible(self):
        with self.assertRaises(OracleError):
            assert_json_equal(b'{"a":1,"b":2}', b'{"b":2,"a":1}')

    def test_duplicate_members_at_depth(self):
        with self.assertRaises(DuplicateJsonMember):
            ordered_json(b'{"nested":[{"a":1,"a":2}]}')

    def test_pairs_preserve_multiplicity_when_inspected(self):
        self.assertNotEqual(ordered_json(b'{"a":1,"a":2}', reject_duplicates=False),
                            ordered_json(b'{"a":2}', reject_duplicates=False))

    def test_object_array_and_boolean_number_remain_distinct(self):
        for left, right in [(b'{}', b'[]'), (b'true', b'1'), (b'false', b'0')]:
            with self.subTest(left=left), self.assertRaises(OracleError):
                assert_json_equal(left, right)

    def test_large_natural_is_exact(self):
        with self.assertRaises(OracleError):
            assert_json_equal(b'9007199254740993', b'9007199254740992')

    def test_newlines_inside_string_are_not_repaired(self):
        with self.assertRaises(OracleError):
            assert_json_equal(b'"a\\r\\nb"', b'"a\\nb"')

    def test_invalid_json_and_encoding(self):
        for raw in [b'{', b'NaN', b'Infinity', b'{}{}', b'"\xff"', b'\xef\xbb\xbf{}']:
            with self.subTest(raw=raw), self.assertRaises(OracleError):
                ordered_json(raw)

    def test_bytes_keep_whitespace_and_cli_removes_one_lf(self):
        with self.assertRaises(OracleError):
            assert_bytes_equal(b'{}\n', b'{}\r\n')
        self.assertEqual(cli_payload(b'{} \n\n'), b'{} \n')
        with self.assertRaises(OracleError):
            cli_payload(b'{}')

    def test_subprocess_capture_keeps_crlf(self):
        proc = run([sys.executable, '-c', 'import sys; sys.stdout.buffer.write(b"a\\r\\nb")'])
        self.assertEqual(proc.stdout, b'a\r\nb')

    def test_controlled_rust_rejection(self):
        assert_rust_rejection(result(err=b'SVP no admitido: InvalidProgram("CellSpec X: b debe ser >= 3")\n'),
                              'bad_b_value')

    def test_internal_panic_signal_and_host_failure_are_not_rejections(self):
        for rc in [0, 2, 101, -11, 137]:
            for check, err, arg in [
                (assert_rust_rejection, b'SVP no admitido: InvalidProgram("b debe ser >= 3")\n', 'bad_b_value')]:
                with self.subTest(rc=rc), self.assertRaises(OracleError):
                    check(result(rc=rc, err=err), arg)

    def test_generic_or_wrong_rejection_is_not_accepted(self):
        for err in [b'', b'\xff', b'ERROR INTERNO: E002\n', b'panic: E002\n', b'ERROR: E004 (InvalidCodomain): b\n']:
            with self.subTest(err=err), self.assertRaises(OracleError):
                assert_rust_rejection(result(err=err), 'bad_b_value')
        with self.assertRaises(OracleError):
            assert_rust_rejection(result(err=b'SVP no admitido: InvalidProgram("otro fallo")\n'), 'bad_b_value')

    def test_rejection_with_ir_is_failure(self):
        with self.assertRaises(OracleError):
            assert_rust_rejection(result(out=b'{}', err=b'SVP no admitido: InvalidProgram("b debe ser >= 3")\n'),
                                  'bad_b_value')

    def test_success_requires_valid_json_and_clean_process(self):
        for proc in [result(rc=0, out=b'not JSON'), result(rc=0, out=b'{}', err=b'error'), result(rc=2, out=b'{}')]:
            with self.subTest(proc=proc), self.assertRaises(OracleError):
                assert_success(proc)


if __name__ == '__main__':
    unittest.main()
