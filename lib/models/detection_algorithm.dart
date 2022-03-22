const String kSharedPreferencesAlgorithmKey = 'algorithm';
enum DetectionAlgorithm { cepstrum, autocorrelation, rawfft }

const String _kCepstrumDescription =
    'The cepstrum pitch detection algorithm works well with instruments that are rich in overtones.';
const String _kAutocorrelation =
    'The autocorrelation pitch detection algorithm works well with instruments that have a pure sound.';
const String _kRawFft =
    'The raw fft pitch detection algorithm works well with most instruments, but may jump between harmonics.';

extension ParseToString on DetectionAlgorithm {
  String toName() {
    switch (index) {
      case 0:
        return 'Power Cepstrum';
      case 1:
        return 'Autocorrelation';
      case 2:
        return 'Raw Fft';
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
      case 2:
        return 'rawfft';
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
      case 2:
        return _kRawFft;
      default:
        return _kCepstrumDescription;
    }
  }

  String instruments() {
    switch (index) {
      case 0:
        return '💰';
      case 1:
        return '✨';
      case 2:
        return '🦗';
      default:
        return '🦗';
    }
  }
}
