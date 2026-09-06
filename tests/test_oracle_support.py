"""Contraejemplos del observador, independientes del resultado del compilador."""
import subprocess
import sys
import unittest

from oracle_support import (OracleError, DuplicateJsonMember, ordered_json,
                            assert_json_equal, assert_bytes_equal, assert_success,
                            assert_python_rejection, assert_rust_rejection,
                            cli_payload, run)


def result(rc=1, out=b"", err=b""):
    return subprocess.CompletedProcess([], rc, out, err)


class OracleTests(unittest.TestCase):
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

    def test_expected_python_diagnostic_not_identifier(self):
        assert_python_rejection(result(err=b'ERROR: E006 (UndeclaredReference): E999\n'), 'E006')
        with self.assertRaises(OracleError):
            assert_python_rejection(result(err=b'ERROR: E999 (Wrong): E006\n'), 'E006')

    def test_controlled_rust_rejection(self):
        assert_rust_rejection(result(err=b'SVP no admitido: InvalidProgram("CellSpec X: b debe ser >= 3")\n'),
                              'bad_b_value')

    def test_internal_panic_signal_and_host_failure_are_not_rejections(self):
        for rc in [0, 2, 101, -11, 137]:
            for check, err, arg in [
                (assert_python_rejection, b'ERROR: E002 (InvalidBValue): b\n', 'E002'),
                (assert_rust_rejection, b'SVP no admitido: InvalidProgram("b debe ser >= 3")\n', 'bad_b_value')]:
                with self.subTest(rc=rc), self.assertRaises(OracleError):
                    check(result(rc=rc, err=err), arg)

    def test_generic_or_wrong_rejection_is_not_accepted(self):
        for err in [b'', b'\xff', b'ERROR INTERNO: E002\n', b'panic: E002\n', b'ERROR: E004 (InvalidCodomain): b\n']:
            with self.subTest(err=err), self.assertRaises(OracleError):
                assert_python_rejection(result(err=err), 'E002')
        with self.assertRaises(OracleError):
            assert_rust_rejection(result(err=b'SVP no admitido: InvalidProgram("otro fallo")\n'), 'bad_b_value')

    def test_rejection_with_ir_is_failure(self):
        with self.assertRaises(OracleError):
            assert_python_rejection(result(out=b'{}', err=b'ERROR: E002 (InvalidBValue): b\n'), 'E002')
        with self.assertRaises(OracleError):
            assert_rust_rejection(result(out=b'{}', err=b'SVP no admitido: InvalidProgram("b debe ser >= 3")\n'),
                                  'bad_b_value')

    def test_success_requires_valid_json_and_clean_process(self):
        for proc in [result(rc=0, out=b'not JSON'), result(rc=0, out=b'{}', err=b'error'), result(rc=2, out=b'{}')]:
            with self.subTest(proc=proc), self.assertRaises(OracleError):
                assert_success(proc)


if __name__ == '__main__':
    unittest.main()
