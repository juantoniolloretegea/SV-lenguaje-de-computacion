"""N0-02: relación constituida y conservación del AST antes del lowering."""
import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'src'))
from svp_errors import SVPError
from svp_lexer import tokenize
from svp_parser import Parser
from svp_validator import Validator

CELL = 'cellspec C { b: 3; codomain: K; semantics: S; role: Base; }'
DOMAIN = 'codomain K = { B, A };'


def ast(source):
    return Parser(tokenize(source, 'n0-02.svp'), 'n0-02.svp').parse()


class OutputSemanticsTotality(unittest.TestCase):
    def test_rechazo_relacional_sin_modificar_ast(self):
        for mappings, reason in [
            ('', 'ausentes=[A, B]'),
            ('A -> "a";', 'ausentes=[B]'),
            ('A -> "a"; B -> "b"; X -> "x";', 'ajenas=[X]'),
            ('A -> "a"; B -> "b"; A -> "a";', 'repetidas=[A]'),
            ('A -> "a"; B -> "b"; A -> "otro";', 'repetidas=[A]'),
        ]:
            for source in [DOMAIN + 'output_semantics S {' + mappings + '}' + CELL,
                           CELL + 'output_semantics S {' + mappings + '}' + DOMAIN]:
                with self.subTest(mappings=mappings, source=source):
                    program = ast(source)
                    before = copy.deepcopy(program)
                    with self.assertRaises(SVPError) as caught:
                        Validator(program).validate()
                    self.assertEqual(caught.exception.error_def.code, 'E115')
                    self.assertIn(reason, str(caught.exception))
                    for identity in ['CellSpec C', 'OutputSemantics S', 'Codomain K']:
                        self.assertIn(identity, str(caught.exception))
                    self.assertEqual(program, before)

    def test_orden_y_textos_compartidos_se_conservan(self):
        program = ast(CELL + 'output_semantics S { A -> "igual"; B -> "igual"; }' + DOMAIN)
        before = copy.deepcopy(program)
        Validator(program).validate()
        self.assertEqual(program, before)

    def test_cada_celda_comprueba_su_propia_relacion(self):
        base = DOMAIN + 'output_semantics S { A -> "a"; B -> "b"; }' + CELL
        Validator(ast(base + CELL.replace('C {', 'D {'))).validate()
        source = base + 'codomain Other = { A, X };' + CELL.replace('C {', 'D {').replace('codomain: K', 'codomain: Other')
        with self.assertRaises(SVPError) as caught:
            Validator(ast(source)).validate()
        self.assertEqual(caught.exception.error_def.code, 'E115')
        self.assertIn('CellSpec D', str(caught.exception))

    def test_otra_semantica_no_completa_la_referenciada(self):
        source = DOMAIN + 'output_semantics S { A -> "a"; } output_semantics Other { B -> "b"; }' + CELL
        with self.assertRaises(SVPError) as caught:
            Validator(ast(source)).validate()
        self.assertEqual(caught.exception.error_def.code, 'E115')

    def test_referencia_semantica_invalida_conserva_e102(self):
        for source in [DOMAIN + CELL, DOMAIN + CELL.replace('semantics: S', 'semantics: K')]:
            with self.subTest(source=source), self.assertRaises(SVPError) as caught:
                Validator(ast(source)).validate()
            self.assertEqual(caught.exception.error_def.code, 'E102')


if __name__ == '__main__':
    unittest.main()
