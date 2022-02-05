import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';

class TunerInherited extends InheritedWidget {
  TunerInherited(this.tunerApi, {Key? key, required Widget child})
      : pitchDetectionStream = tunerApi.initStream(),
        super(key: key, child: child);
  final TunerRs tunerApi;
  final Stream<Partial> pitchDetectionStream;

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
