"""N0-04: identidad real de arquitectura, sin completar ni mutar la entrada."""
import copy
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'src'))
from svp_ast import HorizonDecl
from svp_errors import SVPError
from svp_lexer import tokenize
from svp_parser import Parser
from svp_validator import Validator

BASE = '''codomain K = { A };
output_semantics S { A -> "a"; }
cellspec C { b: 3; codomain: K; semantics: S; role: Base; }
coupledspec CC { cell: C; bridges: [3]; }
semantic_relation R { kind: DeclaredRelation; constraints: [Local]; }
graph G { nodes: [CC]; edges: []; relation: R; regime: Simple; }
'''


def parse(source):
    return Parser(tokenize(source, 'n0-04.svp'), 'n0-04.svp').parse()


class HorizonArchitecture(unittest.TestCase):
    def reject(self, program, code, detail):
        before = copy.deepcopy(program)
        with self.assertRaises(SVPError) as caught:
            Validator(program).validate()
        self.assertEqual(caught.exception.error_def.code, code)
        self.assertIn(detail, caught.exception.detail)
        self.assertEqual(program, before)

    def test_missing_wrong_type_self_and_operation_references(self):
        for prefix, ref, detail in [
            ('', 'Missing', 'Referencia no declarada'),
            ('codomain K = { A };', 'K', 'se esperaba GraphDecl'),
            ('', 'H', 'se esperaba GraphDecl'),
            (BASE + 'cellstate State { spec: C; vector: [U,U,U,U,U,U,U,U,U]; } let E = evaluate(State);',
             'E', 'se esperaba GraphDecl'),
        ]:
            with self.subTest(ref=ref):
                self.reject(parse(prefix + f'horizon H {{ architecture: {ref}; events: [Event]; }}'), 'E006', detail)

    def test_forward_references_preserve_identity_and_event_sequence(self):
        source = 'horizon H { architecture: G; events: [B,A,B]; } horizon Other { architecture: G; events: [C]; }'
        for text in [source + BASE, BASE + source]:
            program = parse(text)
            before = copy.deepcopy(program)
            Validator(program).validate()
            self.assertEqual(program, before)
            horizons = [n for n in program.nodes if isinstance(n, HorizonDecl)]
            self.assertEqual([n.architecture for n in horizons], ['G', 'G'])
            self.assertEqual(horizons[0].events, ['B', 'A', 'B'])

    def test_agent_requires_identity_and_equal_missing_names_do_not_suffice(self):
        source = (ROOT / 'tests/conformance/invalid/agent_arquitecturas_reales_distintas.svp').read_text()
        self.reject(parse(source), 'E402', 'Arch2')
        matching = source.replace('agent AG { architecture: Arch2;', 'agent AG { architecture: Arch1;')
        for reversed_order in [False, True]:
            program = parse(matching)
            if reversed_order:
                program.nodes.reverse()
            before = copy.deepcopy(program)
            Validator(program).validate()
            self.assertEqual(program, before)
            absent = parse(matching.replace('architecture: Arch1;', 'architecture: Missing;'))
            if reversed_order:
                absent.nodes.reverse()
            self.reject(absent, 'E006', 'Missing')

    def test_referenced_graph_must_itself_pass_validation(self):
        source = 'horizon H { architecture: G; events: [Event]; }' + BASE.replace('nodes: [CC]', 'nodes: [S]')
        self.reject(parse(source), 'E006', 'se esperaba CoupledSpecDecl')


if __name__ == '__main__':
    unittest.main()
