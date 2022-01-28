import 'dart:async';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';
import 'package:provider/provider.dart';
import 'package:wakelock/wakelock.dart';

const int tSampleRate = 44000;
const int tNumChannels = 1;
const int tBitsPerSample = 16;
const int tBitRate = tSampleRate * tNumChannels * tBitsPerSample;
const double tMinIntensity = 5000.0;
const double tMaxFrequency = 4186.0;
const double tMinFrequency = 27.5;

// I didn't find a way to set the buffer size, but this seems to be the default
const int tBufferSize = 35200;

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
    Wakelock.enable();
    await _mRecorder!.openAudioSession();
    final tuner = TunerInherited.of(context)!.tunerApi;
    final detectionAlgorithm =
        Provider.of<SettingsModel>(context, listen: false)
            .detectionAlgorithm
            .toShortString();
    await tuner.initTuner(
        algorithm: detectionAlgorithm, numSamples: (tBufferSize / 2).round());
    var recordingDataController = StreamController<Food>();
    _mRecordingDataSubscription =
        recordingDataController.stream.listen((buffer) async {
      if (buffer is FoodData) {
        try {
          final fundamental = await tuner.detectPitch(byteBuffer: buffer.data!);
          if (fundamental.freq > tMinFrequency &&
              fundamental.freq < tMaxFrequency) {
            if ((detectionAlgorithm == 'autocorrelation' &&
                    fundamental.intensity > .9) ||
                (detectionAlgorithm != 'autocorrelation')) {
              Provider.of<PartialsModel>(context, listen: false)
                  .setNewFundamental(fundamental);
            }
          }
        } catch (err) {
          debugPrint('Caught error: $err');
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
