import 'dart:async';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:wakelock/wakelock.dart';
import 'dart:io';
import 'package:permission_handler/permission_handler.dart';

const int tSampleRate = 44000;
const int tNumChannels = 1;
const int tBitsPerSample = 16;
const int tBitRate = tSampleRate * tNumChannels * tBitsPerSample;
const double tMinIntensity = 5000.0;
const double tMaxFrequency = 4186.0;

typedef _Fn = void Function();

class ListenWidget extends StatefulWidget {
  const ListenWidget({
    Key? key,
  }) : super(key: key);

  @override
  _ListenWidgetState createState() => _ListenWidgetState();
}

class _ListenWidgetState extends State<ListenWidget> {
  FlutterSoundRecorder? _mRecorder = FlutterSoundRecorder();
  bool _mRecorderIsInited = false;
  StreamSubscription? _mRecordingDataSubscription;

  Future<void> _openRecorder() async {
    if (Platform.isAndroid) {
      var status = await Permission.microphone.request();
      if (status != PermissionStatus.granted) {
        throw RecordingPermissionException('Microphone permission not granted');
      }
    }
    Wakelock.enable();
    await _mRecorder!.openRecorder();
    var recordingDataController = StreamController<Food>();
    _mRecordingDataSubscription =
        recordingDataController.stream.listen((buffer) async {
      if (buffer is FoodData) {
        context.read<TunerBloc>().add(TunerEvent.bufferReady(buffer.data!));
      }
    });
    await _mRecorder!.startRecorder(
      toStream: recordingDataController.sink,
      codec: Codec.pcm16,
      numChannels: tNumChannels,
      bitRate: tBitRate,
      sampleRate: tSampleRate,
    );
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
    _mRecorder!.closeRecorder();
    _mRecorder = null;
    super.dispose();
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
    return _mRecorder!.isPaused
        ? () => _mRecorder!.resumeRecorder().then((value) => setState(() {}))
        : () => _mRecorder!.pauseRecorder().then((value) => setState(() {}));
  }

  void onListenPressed() {
    if (!_mRecorderIsInited) {
      return;
    }
    final isPaused = _mRecorder!.isPaused;
    Wakelock.toggle(enable: !isPaused);
    if (isPaused) {
      _mRecorder!.resumeRecorder().then((value) => setState(() {}));
    } else {
      _mRecorder!.pauseRecorder().then((value) => setState(() {}));
    }
  }

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: () {
        getRecorderFn()!();
      },
      tooltip: _mRecorder!.isRecording ? 'Listening...' : 'Not listening...',
      child: Icon(
          _mRecorder!.isRecording ? Icons.mic_outlined : Icons.mic_off_sharp),
    );
  }
}
