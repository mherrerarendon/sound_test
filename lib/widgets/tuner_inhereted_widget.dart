import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'dart:ffi';
import 'dart:io';

class TunerInherited extends InheritedWidget {
  TunerInherited({Key? key, required Widget child})
      : super(key: key, child: child);

  final TunerRs tunerApi = TunerRs(Platform.isAndroid
      ? DynamicLibrary.open('libtuner_rs.so')
      : DynamicLibrary.process());

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
