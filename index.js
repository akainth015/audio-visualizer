const fs = require("fs");

const output = fs.createWriteStream("../../OneDrive/Documents/Rainmeter/Skins/AudioVisualizer/AudioVisualizer_Flipped.ini");

// language=Ini
const rainmeter = `
[Rainmeter]
Update = 40
DynamicWindowSize = 1
AccurateText = 1
`;

output.write(rainmeter);

const NUM_BANDS = 30;

// language=Ini
const audioMeasure = `
[MeasureAudio]
Measure=Plugin
Plugin=AudioLevel
FFTSize=1024
FFTAttack=15
FFTDecay=300
Bands=${NUM_BANDS}
`;

output.write(audioMeasure);

for (let i = 0; i < NUM_BANDS; i++) {
    // language=Ini
    const bandMeasure = `
[MeasureBand${i}]
Measure=Plugin
Plugin=AudioLevel
Parent=MeasureAudio
Type=Band
BandIdx=${i}
`;
    output.write(bandMeasure);
}

for (let i = 0; i < NUM_BANDS; i++) {
    // language=Ini
    const bandMeter = `
[MeterBandL${i}]
Meter=Bar
MeasureName=MeasureBand${i}
Flip=1
X=5R
Y=0r
W=5
H=100
BarColor=255,255,255
BarOrientation=Vertical
`;
    output.write(bandMeter);
}

for (let i = NUM_BANDS - 1; i >= 0; i--) {
    // language=Ini
    const bandMeter = `
[MeterBandR${i}]
Meter=Bar
MeasureName=MeasureBand${i}
Flip=1
X=5R
Y=0r
W=5
H=100
BarColor=255,255,255
BarOrientation=Vertical
`;
    output.write(bandMeter);
}