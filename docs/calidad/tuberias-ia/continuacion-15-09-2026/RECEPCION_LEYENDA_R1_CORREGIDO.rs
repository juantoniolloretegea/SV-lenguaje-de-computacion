//! Regresión receptora del reparo R1 de leyenda: orden de máscaras.
//! Receiver regression: mask order, using the unchanged published library.
// Compilar externamente contra la biblioteca del corte 6d73c376.
// No ejecuta reconocer, PNG, Q1/Q2 ni E1-E16.
use leyenda_contenido::atribucion::{atribuir, mapa_exclusivo, union_geometrica, Atribucion};
use leyenda_contenido::parametros::Parametros;
use leyenda_contenido::plantillas::Mascara;
fn parametros(epsilon: f64) -> Parametros {
    Parametros::analizar(&format!("n_min=1\ntheta=0.5\nepsilon={epsilon}\nrho=0.1\nr_max=10\na_min=4\ns_px=12.8\ntau_t=1\nd_min=1\ng_min=1\nw_sep_min=1\nw_sep_max=6\npaso_x=1\nmax_rasterizaciones=10\nmax_pixeles_mascara=100\n")).unwrap()
}
fn mascara(s: f64, pix: &[(u32,u32)]) -> Mascara {
    Mascara { texto: "t".into(), origen_x: 0, origen_y: 334, puntuacion: s, pixeles: pix.iter().copied().collect() }
}
fn main() {
    let orders = [[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
    let p = parametros(0.02);
    let scores = [0.95, 0.80, 0.79];
    for (i, order) in orders.iter().enumerate() {
        let masks: Vec<_> = order.iter().map(|&k| mascara(scores[k], &[(11,320)])).collect();
        assert_eq!(atribuir(&masks, &p), Atribucion::Empate);
        assert_eq!(mapa_exclusivo(&masks, &p), Err(Atribucion::Empate));
        println!("contraejemplo_historico permutacion={} resultado=Empate", i+1);
    }
    let p = parametros(0.05);
    let masks = [
        mascara(0.70, &[(11,320),(10,320)]),
        mascara(0.95, &[(11,320)]),
        mascara(0.80, &[(11,320),(12,320)]),
    ];
    for (i, order) in orders.iter().enumerate() {
        let m: Vec<_> = order.iter().map(|&k| masks[k].clone()).collect();
        assert_eq!(atribuir(&m, &p), Atribucion::Exclusiva);
        let map = mapa_exclusivo(&m, &p).unwrap();
        let winner = map[&(11,320)];
        assert_eq!(m[winner].puntuacion, 0.95);
        assert_eq!(union_geometrica(&m).len(), 3);
        println!("control_sin_empate permutacion={} ganador=0.95 union=3", i+1);
    }
    println!("conforme=12/12; alcance=funciones_publicas_con_mascaras_sinteticas");
}
