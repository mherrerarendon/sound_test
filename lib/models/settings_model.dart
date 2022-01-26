import 'package:flutter/material.dart';

enum DetectionAlgorithm {
  marco,
  cepstrum,
  autocorrelation,
}

extension ParseToString on DetectionAlgorithm {
  String toName() {
    switch (index) {
      case 0:
        return 'Marco';
      case 1:
        return 'Cepstrum';
      case 2:
        return 'Autocorrelation';
      default:
        return 'Marco';
    }
  }

  String toShortString() {
    switch (index) {
      case 0:
        return 'marco';
      case 1:
        return 'cepstrum';
      case 2:
        return 'autocorrelation';
      default:
        return 'marco';
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
