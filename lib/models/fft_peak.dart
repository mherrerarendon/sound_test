import 'package:flutter/foundation.dart';

class FftPeakModel extends ChangeNotifier {
  double _freq = 0.0;
  double _intensity = 0.0;

  double get freq => _freq;
  double get intensity => _intensity;

  void setNewPeak(double freq, double intensity) {
    _freq = freq;
    _intensity = intensity;
    notifyListeners();
  }
}
