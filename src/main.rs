const NUM_BANDS: i32 = 30;

const BAR_WIDTH: i32 = 5;
const MARGIN: i32 = 5;

const MAX_X: i32 = 2 * NUM_BANDS * (BAR_WIDTH + MARGIN);

fn main() {
    println!("[Metadata]");
    println!("Name=Pulse");
    println!("Author=Aanand Kainth");
    println!("Information=Shows an audio visualizer based on speaker output");
    println!("License=MIT");
    println!("Version=1.0.0");
    println!();

    println!("[Rainmeter]");
    println!("Update=40");
    println!();

    println!("[MeasureAudio]");
    println!("Measure=Plugin");
    println!("Plugin=AudioLevel");
    println!("FFTSize=1024");
    println!("FFTAttack=15");
    println!("FFTDecay=250");
    println!("Bands={}", NUM_BANDS);
    println!();

    for i in 0..NUM_BANDS {
        println!("[MeasureBand{}]", i);
        println!("Measure=Plugin");
        println!("Plugin=AudioLevel");
        println!("Parent=MeasureAudio");
        println!("Type=Band");
        println!("BandIdx={}", i);
        println!();

        let left_x = i * (BAR_WIDTH + MARGIN);

        println!("[MeterBandLT{}]", i);
        println!("Meter=Bar");
        println!("MeasureName=MeasureBand{}", i);
        println!("X={}", left_x);
        println!("Y=0");
        println!("W={}", BAR_WIDTH);
        println!("H=100");
        println!("BarColor=255,255,255");
        println!("BarOrientation=Vertical");
        println!();

        println!("[MeterBandLB{}]", i);
        println!("Meter=Bar");
        println!("MeasureName=MeasureBand{}", i);
        println!("Flip=1");
        println!("X={}", left_x);
        println!("Y=100");
        println!("W={}", BAR_WIDTH);
        println!("H=100");
        println!("BarColor=255,255,255");
        println!("BarOrientation=Vertical");
        println!();

        let right_x = MAX_X - left_x;

        println!("[MeterBandRT{}]", i);
        println!("Meter=Bar");
        println!("MeasureName=MeasureBand{}", i);
        println!("X={}", right_x);
        println!("Y=0");
        println!("W={}", BAR_WIDTH);
        println!("H=100");
        println!("BarColor=255,255,255");
        println!("BarOrientation=Vertical");
        println!();

        println!("[MeterBandRB{}]", i);
        println!("Meter=Bar");
        println!("MeasureName=MeasureBand{}", i);
        println!("Flip=1");
        println!("X={}", right_x);
        println!("Y=100");
        println!("W={}", BAR_WIDTH);
        println!("H=100");
        println!("BarColor=255,255,255");
        println!("BarOrientation=Vertical");
        println!();
    }
}
