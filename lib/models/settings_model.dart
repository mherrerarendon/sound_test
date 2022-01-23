import 'package:flutter/material.dart';

enum DetectionAlgorithm {
  Marco,
  Cepstrum,
}

extension ParseToString on DetectionAlgorithm {
  String toName() {
    switch (index) {
      case 0:
        return 'Marco';
      case 1:
        return 'Cepstrum';
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
      default:
        return 'marco';
    }
  }
}

class SettingsModel extends ChangeNotifier {
  DetectionAlgorithm _detectionAlgorithm = DetectionAlgorithm.Marco;

  DetectionAlgorithm get detectionAlgorithm => _detectionAlgorithm;
  void setDetectionAlgorithm(DetectionAlgorithm value) {
    _detectionAlgorithm = value;
    notifyListeners();
  }
}
