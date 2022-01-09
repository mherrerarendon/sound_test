import 'dart:math';

import 'package:flutter/foundation.dart';

const kA4Freq = 440.0;

class FftPeakModel extends ChangeNotifier {
  double _freq = 0.0;
  double _intensity = 0.0;

  double get freq => _freq;
  double get intensity => _intensity;

  double get stepsFromA4 => 12 * log(_freq / kA4Freq) / log(2);

  void setNewPeak(double freq, double intensity) {
    _freq = freq;
    _intensity = intensity;
    notifyListeners();
  }

  // double getStepsFromA4() {
  //   return 12 * log(_freq / kA4Freq) / log(2);
  // }
}
