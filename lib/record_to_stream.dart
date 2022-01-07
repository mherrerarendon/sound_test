/*
 * Copyright 2018, 2019, 2020, 2021 Dooboolab.
 *
 * This file is part of Flutter-Sound.
 *
 * Flutter-Sound is free software: you can redistribute it and/or modify
 * it under the terms of the Mozilla Public License version 2 (MPL2.0),
 * as published by the Mozilla organization.
 *
 * Flutter-Sound is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * MPL General Public License for more details.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import 'dart:async';
import 'dart:ffi';
import 'package:flutter/material.dart';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:sound_test/api.dart';
// import 'package:permission_handler/permission_handler.dart';

/*
 * This is an example showing how to record to a Dart Stream.
 * It writes all the recorded data from a Stream to a File, which is completely stupid:
 * if an App wants to record something to a File, it must not use Streams.
 *
 * The real interest of recording to a Stream is for example to feed a
 * Speech-to-Text engine, or for processing the Live data in Dart in real time.
 *
 */

///
const int tSampleRate = 44000;
const int tNumChannels = 1;
const int tBitsPerSample = 16;
const int tBitRate = tSampleRate * tNumChannels * tBitsPerSample;
const double tMinIntensity = 10000.0;
const double tMaxFrequency = 20000.0;
const double tMinFrequency = 0.0;
typedef _Fn = void Function();

/// Example app.
class TunerWidget extends StatefulWidget {
  @override
  _TunerWidgetState createState() => _TunerWidgetState();
}

class _TunerWidgetState extends State<TunerWidget> {
  FlutterSoundRecorder? _mRecorder = FlutterSoundRecorder();
  bool _mRecorderIsInited = false;
  StreamSubscription? _mRecordingDataSubscription;
  final _tuner = TunerRs(DynamicLibrary.process());
  FftPeak _peak = FftPeak(freq: 0, intensity: 0);

  Future<void> _openRecorder() async {
    // var status = await Permission.microphone.request();
    // if (status != PermissionStatus.granted) {
    //   throw RecordingPermissionException('Microphone permission not granted');
    // }
    await _mRecorder!.openAudioSession();
    setState(() {
      _mRecorderIsInited = true;
    });
  }

  @override
  void initState() {
    super.initState();
    _openRecorder();
  }

  @override
  void dispose() {
    stopRecorder();
    _mRecorder!.closeAudioSession();
    _mRecorder = null;
    super.dispose();
  }

  // ----------------------  Here is the code to record to a Stream ------------

  Future<void> record() async {
    assert(_mRecorderIsInited);
    var recordingDataController = StreamController<Food>();
    _mRecordingDataSubscription =
        recordingDataController.stream.listen((buffer) async {
      if (buffer is FoodData) {
        final newPeak = await _tuner.fft(byteBuffer: buffer.data!);
        if (newPeak.intensity > tMinIntensity &&
            newPeak.freq > tMinFrequency &&
            newPeak.freq < tMaxFrequency) {
          _peak = newPeak;
          setState(() {});
        }
      }
    });
    await _mRecorder!.startRecorder(
      toStream: recordingDataController.sink,
      codec: Codec.pcm16,
      numChannels: tNumChannels,
      bitRate: tBitRate,
      sampleRate: tSampleRate,
    );
    setState(() {});
  }

  Future<void> stopRecorder() async {
    await _mRecorder!.stopRecorder();
    if (_mRecordingDataSubscription != null) {
      await _mRecordingDataSubscription!.cancel();
      _mRecordingDataSubscription = null;
    }
  }

  _Fn? getRecorderFn() {
    if (!_mRecorderIsInited) {
      return null;
    }
    return _mRecorder!.isStopped
        ? record
        : () {
            stopRecorder().then((value) => setState(() {}));
          };
  }

  // ----------------------------------------------------------------------------------------------------------------------

  @override
  Widget build(BuildContext context) {
    Widget makeBody() {
      return Column(
        children: [
          Container(
            margin: const EdgeInsets.all(3),
            padding: const EdgeInsets.all(3),
            height: 80,
            width: double.infinity,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: Color(0xFFFAF0E6),
              border: Border.all(
                color: Colors.indigo,
                width: 3,
              ),
            ),
            child: ElevatedButton(
              onPressed: getRecorderFn(),
              child: Text(_mRecorder!.isRecording ? 'Stop' : 'Listen'),
            ),
          ),
          Container(
            margin: const EdgeInsets.all(3),
            padding: const EdgeInsets.all(3),
            height: 80,
            width: double.infinity,
            alignment: Alignment.centerLeft,
            decoration: BoxDecoration(
              color: Color(0xFFFAF0E6),
              border: Border.all(
                color: Colors.indigo,
                width: 3,
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text('Frequency: ${_peak.freq.toStringAsFixed(2)}'),
            ),
          ),
          Container(
            margin: const EdgeInsets.all(3),
            padding: const EdgeInsets.all(3),
            height: 80,
            width: double.infinity,
            alignment: Alignment.centerLeft,
            decoration: BoxDecoration(
              color: Color(0xFFFAF0E6),
              border: Border.all(
                color: Colors.indigo,
                width: 3,
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text('Intensity: ${_peak.intensity.toStringAsFixed(2)}'),
            ),
          ),
        ],
      );
    }

    return Scaffold(
      backgroundColor: Colors.blue,
      appBar: AppBar(
        title: const Text('Sound Test'),
      ),
      body: makeBody(),
    );
  }
}
