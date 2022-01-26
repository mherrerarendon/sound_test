import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:sound_test/api.dart';

const kA4Freq = 440.0;

// 15 cents (Noticable pitch difference starts at around 10-25 cents)
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

class PartialsModel extends ChangeNotifier {
  List<Partial> _partials = [Partial(freq: 1.0, intensity: 1.0)];

  double get freq => _partials.first.freq;
  double get intensity => _partials.first.intensity;
  int get numPartials => _partials.length;

  double get stepsFromA4 => 12 * log(freq / kA4Freq) / log(2);
  String get noteName =>
      kNotes[((stepsFromA4 + 4 * 12).round() % 12.0).toInt()];
  double get centsOffset => (stepsFromA4 - stepsFromA4.round()) * 100;

  bool inTune() {
    final double absoluteCentsOffset = centsOffset.abs();
    return (absoluteCentsOffset < kMaxCentsOffset);
  }

  void setNewPartials(List<Partial> partials) {
    _partials.clear();
    _partials = partials;
    notifyListeners();
  }

  Partial getPartial(int index) {
    return _partials[index];
  }
}
