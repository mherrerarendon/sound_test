import 'package:flutter/material.dart';

enum DetectionAlgorithm {
  power,
  autocorrelation,
}

const String _kCepstrumDescription =
    'The cepstrum pitch detection algorithm works well with instruments that are rich in overtones.';
const String _kAutocorrelation =
    'The autocorrelation pitch detection algorithm works well with instruments that have a pure sound.';

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

  String description() {
    switch (index) {
      case 0:
        return _kCepstrumDescription;
      case 1:
        return _kAutocorrelation;
      default:
        return _kCepstrumDescription;
    }
  }

  String instruments() {
    switch (index) {
      case 0:
        return '🎻🎺🎷';
      case 1:
        return '🎻🎺🎷';
      default:
        return _kCepstrumDescription;
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
