import 'dart:async';
import 'dart:ffi';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:sound_test/api.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/models/fft_peak.dart';
import 'package:provider/provider.dart';

const int tSampleRate = 44000;
const int tNumChannels = 1;
const int tBitsPerSample = 16;
const int tBitRate = tSampleRate * tNumChannels * tBitsPerSample;
const double tMinIntensity = 3000.0;
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
        debugPrint('Intensity: ${newPeak.intensity}');
        if (newPeak.intensity > tMinIntensity &&
            newPeak.freq > tMinFrequency &&
            newPeak.freq < tMaxFrequency) {
          Provider.of<FftPeakModel>(context, listen: false)
              .setNewPeak(newPeak.freq, newPeak.intensity);
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

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(3),
      padding: const EdgeInsets.all(3),
      height: 80,
      width: double.infinity,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: const Color(0xFFFAF0E6),
        border: Border.all(
          color: Colors.indigo,
          width: 3,
        ),
      ),
      child: ElevatedButton(
        onPressed: getRecorderFn(),
        child: Text(_mRecorder!.isRecording ? 'Stop' : 'Listen'),
      ),
    );
  }
}
