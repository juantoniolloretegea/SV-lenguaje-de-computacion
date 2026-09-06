"""N0-03: multiplicidad preservada desde AST y mapas independientes."""
import copy
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'src'))
from svp_ast import OutputSemanticsDecl, ConnectorDecl
from svp_errors import SVPError
from svp_lexer import tokenize
from svp_parser import Parser
from svp_validator import Validator
from svp_ir import Lowering
from svp_serialize import serialize
from oracle_support import assert_json_roundtrip


def ast(source):
    return Parser(tokenize(source, 'n0-03.svp'), 'n0-03.svp').parse()


class JsonProjection(unittest.TestCase):
    def test_unbound_duplicate_is_rejected_before_lowering_without_mutation(self):
        for second in ['igual', 'distinto']:
            program = ast(f'output_semantics S {{ A -> "igual"; A -> "{second}"; }}')
            before = copy.deepcopy(program)
            with self.assertRaises(SVPError) as caught:
                Validator(program).validate()
            self.assertEqual(caught.exception.error_def.code, 'E115')
            self.assertEqual(caught.exception.detail, 'OutputSemantics S: repetidas=[A]')
            self.assertEqual(program, before)

    def test_multiple_maps_keep_their_own_members_and_empty_map(self):
        source = 'output_semantics First { A -> "uno"; a -> "uno"; } output_semantics Second { A -> "dos"; } output_semantics Empty {}'
        program = ast(source)
        before = copy.deepcopy(program)
        Validator(program).validate()
        raw = serialize(Lowering(program).lower()).encode()
        doc = json.loads(assert_json_roundtrip(raw))
        self.assertEqual([o['fields']['mappings'] for o in doc['objects']],
                         [{'A': 'uno', 'a': 'uno'}, {'A': 'dos'}, {}])
        self.assertEqual(program, before)

    def test_every_corpus_dynamic_map_preserves_source_members(self):
        sources = sorted((ROOT / 'tests/conformance/valid').glob('*.svp'))
        self.assertTrue(sources)
        families = set()
        for path in sources:
            with self.subTest(path=path.name):
                program = ast(path.read_bytes().decode('utf-8'))
                Validator(program).validate()
                raw = serialize(Lowering(program).lower()).encode()
                objects = {obj['name']: obj for obj in json.loads(assert_json_roundtrip(raw))['objects']}
                for node in program.nodes:
                    if isinstance(node, OutputSemanticsDecl):
                        pairs, field = node.mappings, 'mappings'
                    elif isinstance(node, ConnectorDecl):
                        pairs, field = node.mapping, 'mapping'
                    else:
                        continue
                    families.add(field)
                    actual = objects[node.name]['fields'][field]
                    self.assertEqual(len(actual), len(pairs))
                    for key, value in pairs:
                        self.assertEqual(actual[key], value)
        self.assertEqual(families, {'mappings', 'mapping'})


if __name__ == '__main__':
    unittest.main()
