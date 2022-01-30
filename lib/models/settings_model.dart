import 'package:flutter/material.dart';

enum DetectionAlgorithm {
  power,
  autocorrelation,
}

extension ParseToString on DetectionAlgorithm {
  String toName() {
    switch (index) {
      case 0:
        return 'Power Cepstrum';
      case 1:
        return 'Autocorrelation';
      default:
        return 'Power Cestrum';
    }
  }

  String toShortString() {
    switch (index) {
      case 0:
        return 'power';
      case 1:
        return 'autocorrelation';
      default:
        return 'power';
    }
  }
}

class SettingsModel extends ChangeNotifier {
  DetectionAlgorithm _detectionAlgorithm = DetectionAlgorithm.autocorrelation;

  DetectionAlgorithm get detectionAlgorithm => _detectionAlgorithm;
  void setDetectionAlgorithm(DetectionAlgorithm value) {
    _detectionAlgorithm = value;
    notifyListeners();
  }
}
