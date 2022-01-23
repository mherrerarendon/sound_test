import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'dart:ffi';

class TunerInherited extends InheritedWidget {
  TunerInherited({Key? key, required Widget child})
      : super(key: key, child: child);

  final TunerRs tunerApi = TunerRs(DynamicLibrary.process());

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
