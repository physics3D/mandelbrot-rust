static bildName: &str = "mandelbrot.png";
static breite: u32 = 1920;
static hoehe: u32 = 1080;
static maxIterationen: i64 = 1000;
static realteilMittelpunkt: f64 = 0.0;
static imaginaerteilMittelpunkt: f64 = 0.0;
static zoom: i64 = 500;

fn konvertierePixelZuKoordinaten(pixel: [f64; 2]) -> [f64; 2] {
    let realteil = ((pixel[0] - breite as f64 / 2.0) / zoom as f64) + realteilMittelpunkt;
    let imaginaerteil = ((pixel[1] - hoehe as f64 / 2.0) / zoom as f64) + imaginaerteilMittelpunkt;

    return [realteil, imaginaerteil];
}

fn mandelbrot(komplexeZahl: [f64; 2]) -> i64 {
    let mut cx = komplexeZahl[0];
    let mut cy = komplexeZahl[1];
    let mut iterationen = 0;

    let mut cx2 = cx * cx;
    let mut cy2 = cy * cy;
    let mut cxy = cx * cy;

    while iterationen < maxIterationen && cx2 + cy2 <= 4 as f64 {
        cx = cx2 - cy2 + komplexeZahl[0];
        cy = cxy + cxy + komplexeZahl[1];

        cx2 = cx * cx;
        cy2 = cy * cy;
        cxy = cx * cy;

        iterationen += 1;
    }

    return iterationen;
}

fn konvertiereIterationenZuFarbe(gezaehlteIterationen: i64) -> image::Rgb<u8> {
    let ergebnis = (gezaehlteIterationen as f64 / maxIterationen as f64 * 255.0) as u8;
    return image::Rgb([ergebnis, ergebnis, ergebnis]);
}

fn main() {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(breite, hoehe);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let pos = [x as f64, y as f64];
        let koordinaten = konvertierePixelZuKoordinaten(pos);
        let iterationen = mandelbrot(koordinaten);
        let farbe = konvertiereIterationenZuFarbe(iterationen);
        *pixel = farbe;
    }

    imgbuf.save(bildName).unwrap();
}
