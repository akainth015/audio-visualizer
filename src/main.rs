const NUM_BANDS: i32 = 35;

const BAR_WIDTH: i32 = 5;
const MARGIN: i32 = 8;

const MAX_X: i32 = 2 * NUM_BANDS * (BAR_WIDTH + MARGIN) - MARGIN;

fn main() {
    println!("\
    [Metadata]
    Name=Pulse
    Author=Aanand Kainth
    Information=Shows an audio visualizer based on speaker output
    License=MIT
    Version=2.0.2

    [Rainmeter]
    Update=40
    SkinHeight=200
    
    [MeasureAudio]
    Measure=Plugin
    Plugin=AudioLevel
    FFTSize=1024
    FFTAttack=250
    FFTDecay=250
    Bands={}
    ", NUM_BANDS);

    for i in 0..NUM_BANDS {
        println!("\
        [MeasureBand{0}]
        Measure=Plugin
        Plugin=AudioLevel
        Parent=MeasureAudio
        Type=Band
        BandIdx={0}
        ", i);

        let left_x = i * (BAR_WIDTH + MARGIN);

        println!("\
        [MeterBandLT{0}]
        Meter=Shape
        Shape=Ellipse {1},(100+{2}-100 * [MeasureBand{0}]), ({2} * 100 * [MeasureBand{0}] / 100) | StrokeWidth 0
        DynamicVariables=1
        
        ", i, left_x, BAR_WIDTH);

        println!("\
        [MeterBandLB{0}]
        Meter=Shape
        Shape=Ellipse {1},(100+{2}+60 * [MeasureBand{0}]), ({2} * 100 * [MeasureBand{0}] / 130) | StrokeWidth 0
        DynamicVariables=1
        
        ", i, left_x, BAR_WIDTH);

        let right_x = MAX_X - left_x;

        println!("\
        [MeterBandRT{0}]
        Meter=Shape
        Shape=Ellipse {1},(100+{2}-100 * [MeasureBand{0}]), ({2} * 100 * [MeasureBand{0}] / 100) | StrokeWidth 0
        DynamicVariables=1
        
        ", i, right_x, BAR_WIDTH);

        println!("\
        [MeterBandRB{0}]
        Meter=Shape
        Shape=Ellipse {1},(100+{2}+60 * [MeasureBand{0}]), ({2} * 100 * [MeasureBand{0}] / 130) | StrokeWidth 0
        DynamicVariables=1
        
        ", i, right_x, BAR_WIDTH);
    }
}
