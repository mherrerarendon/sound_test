import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:sound_test/api.dart';

const kA4Freq = 440.0;

// 15 cents (Noticable pitch difference starts at around 10-25 cents)
const kMaxCentsOffset = 15.0;

class PartialsModel extends ChangeNotifier {
  List<Partial> _partials = [Partial(freq: 1.0, intensity: 1.0)];

  double get freq => _partials.first.freq;
  double get intensity => _partials.first.intensity;
  int get numPartials => _partials.length;

  double get stepsFromA4 => 12 * log(freq / kA4Freq) / log(2);

  bool inTune() {
    final stepsFromA4 = this.stepsFromA4;
    final int closestNote = stepsFromA4.round();
    final double centsOffset = ((stepsFromA4 - closestNote) * 100.0).abs();
    return (centsOffset < kMaxCentsOffset);
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
