import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'dart:ffi';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sound_test/models/settings_model.dart';

class TunerInherited extends InheritedWidget {
  TunerInherited({Key? key, required Widget child})
      : super(key: key, child: child) {
    WidgetsFlutterBinding.ensureInitialized();
    tunerApi = TunerRs(DynamicLibrary.process());
    SharedPreferences.getInstance().then((prefs) {
      pitchDetectionStream = tunerApi.initTuner(
          algorithm: DetectionAlgorithm
              .values[prefs.getInt(kSharedPreferencesAlgorithmKey)!]
              .toShortString());
    });
  }
  late TunerRs tunerApi;
  late Stream<Partial> pitchDetectionStream;

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
