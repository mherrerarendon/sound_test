import 'dart:async';
import 'dart:ffi';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:sound_test/api.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:provider/provider.dart';

const int tSampleRate = 44000;
const int tNumChannels = 1;
const int tBitsPerSample = 16;
const int tBitRate = tSampleRate * tNumChannels * tBitsPerSample;
const double tMinIntensity = 5000.0;
const double tMaxFrequency = 4186.0;
const double tMinFrequency = 27.5;
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
  final _tuner = TunerRs(DynamicLibrary.process());

  Future<void> _openRecorder() async {
    await _mRecorder!.openAudioSession();
    var recordingDataController = StreamController<Food>();
    _mRecordingDataSubscription =
        recordingDataController.stream.listen((buffer) async {
      if (buffer is FoodData) {
        final harmonicPartials = await _tuner.fft(byteBuffer: buffer.data!);
        final fundamental = harmonicPartials[0];
        if (fundamental.intensity > tMinIntensity &&
            fundamental.freq > tMinFrequency &&
            fundamental.freq < tMaxFrequency) {
          Provider.of<PartialsModel>(context, listen: false)
              .setNewPartials(harmonicPartials);
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

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: getRecorderFn(),
      child: Icon(
          _mRecorder!.isRecording ? Icons.mic_outlined : Icons.mic_off_sharp),
    );
  }
}
