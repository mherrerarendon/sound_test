import 'dart:math';

import 'package:sound_test/api.dart';

const kA4Freq = 440.0;

// Noticable pitch difference starts at around 10-25 cents
const kMaxCentsOffset = 10.0;

const kNotes = [
  'A',
  'A#',
  'B',
  'C',
  'C#',
  'D',
  'D#',
  'E',
  'F',
  'F#',
  'G',
  'G#',
];
const kA4Index = 12 * 4;

class PartialsModel {
  PartialsModel(this._partial);
  final Partial _partial;

  double get freq => _partial.freq;
  double get intensity => _partial.intensity;

  double get stepsFromA4 => 12 * log(freq / kA4Freq) / log(2);
  String get noteName =>
      kNotes[((stepsFromA4 + 4 * 12).round() % 12.0).toInt()];
  String get leftNoteName =>
      kNotes[((stepsFromA4 + 5 * 12 - 1).round() % 12.0).toInt()];
  String get rigthNoteName =>
      kNotes[((stepsFromA4 + 4 * 12 + 1).round() % 12.0).toInt()];
  double get centsOffset => (stepsFromA4 - stepsFromA4.round()) * 100;

  bool inTune() {
    final double absoluteCentsOffset = centsOffset.abs();
    return (absoluteCentsOffset < kMaxCentsOffset);
  }
}
