import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'dart:ffi';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sound_test/models/settings_model.dart';

class TunerInherited extends InheritedWidget {
  TunerInherited(this.tunerApi, {Key? key, required Widget child})
      : pitchDetectionStream = tunerApi.initStream(),
        super(key: key, child: child);
  final TunerRs tunerApi;
  final Stream<Partial> pitchDetectionStream;

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  Stream<Partial> pitchStream() async* {
    // Stores any partial line from the previous chunk.
    // var partial = '';
    // Wait until a new chunk is available, then process it.
    await for (final pitch in pitchDetectionStream) {
      debugPrint('pitch: $pitch');
      yield pitch;
      // var lines = chunk.split('\n');
      // lines[0] = partial + lines[0]; // Prepend partial line.
      // partial = lines.removeLast(); // Remove new partial line.
      // for (final line in lines) {
      //   yield line; // Add lines to output stream.
      // }
    }
    // Add final partial line to output stream, if any.
    // if (partial.isNotEmpty) yield partial;
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
