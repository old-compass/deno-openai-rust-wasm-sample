// // src/lib.rs

// TODO:
// simd動くか試したい
// simd版とno simd版のライブラリにしたいよね
// cloudflare workers(rst(wasm))でも動くかみたい

#[no_mangle]
pub extern "C" fn cosine_similarity() {

}

// // コサイン類似度を計算する関数
// #[no_mangle]
// pub extern "C" fn cosine_similarity(a_ptr: *const f64, b_ptr: *const f64, len: usize) -> f64 {
//     let a = unsafe { std::slice::from_raw_parts(a_ptr, len) };
//     let b = unsafe { std::slice::from_raw_parts(b_ptr, len) };
    
//     let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
//     let a_magnitude: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
//     let b_magnitude: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
//     if a_magnitude == 0.0 || b_magnitude == 0.0 {
//         return 0.0; // 大きさが0のベクトルの場合はコサイン類似度が定義されない
//     }
    
//     dot_product / (a_magnitude * b_magnitude)
// }

