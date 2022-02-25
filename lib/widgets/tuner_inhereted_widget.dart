import 'dart:io';
import 'dart:async';
import 'dart:math';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'package:sound_test/constants.dart';

class TunerInherited extends InheritedWidget {
  TunerInherited(this.tunerApi, {Key? key, required Widget child})
      : super(key: key, child: child);
  final TunerRs tunerApi;
  final StreamController<Partial> _controller = StreamController<Partial>();
  final Uint8List _buffer = Uint8List(tBufferSize);
  int _bufferCursor = 0;

  get stream => _controller.stream;
  void addData(Uint8List data) async {
    if (Platform.isAndroid) {
      final remainder = tBufferSize - _bufferCursor;
      final dataEnd = min(remainder, data.length);
      _copyData(data.sublist(0, dataEnd));

      if (_bufferCursor >= tBufferSize) {
        try {
          final partial =
              await tunerApi.detectPitchWithBuffer(byteBuffer: _buffer);
          _controller.add(partial);
          _resetBuffer();
        } catch (e) {
          _resetBuffer();
        }
      }
    } else {
      try {
        final partial = await tunerApi.detectPitchWithBuffer(byteBuffer: data);
        _controller.add(partial);
      } catch (e) {
        print(e);
      }
    }
  }

  void _resetBuffer() {
    _buffer.fillRange(0, tBufferSize, 0);
    _bufferCursor = 0;
  }

  void _copyData(Uint8List data) {
    for (final dat in data) {
      _buffer[_bufferCursor] = dat;
      _bufferCursor++;
    }
  }

  static TunerInherited? of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<TunerInherited>();
  }

  @override
  bool updateShouldNotify(TunerInherited oldWidget) {
    return false;
  }
}
