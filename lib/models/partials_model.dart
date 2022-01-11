import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:sound_test/api.dart';

const kA4Freq = 440.0;

class PartialsModel extends ChangeNotifier {
  List<Partial> _partials = [Partial(freq: 0.0, intensity: 0.0)];
  // double _freq = 0.0;
  // double _intensity = 0.0;

  double get freq => _partials.first.freq;
  double get intensity => _partials.first.intensity;
  int get numPartials => _partials.length;

  double get stepsFromA4 => 12 * log(freq / kA4Freq) / log(2);

  void setNewPartials(List<Partial> partials) {
    _partials.clear();
    _partials = partials;
    notifyListeners();
  }

  Partial getPartial(int index) {
    return _partials[index];
  }

  // void setNewPeak(double freq, double intensity) {
  //   _freq = freq;
  //   _intensity = intensity;
  //   notifyListeners();
  // }
}
