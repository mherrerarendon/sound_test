import 'dart:typed_data';

import 'package:flutter_audio_capture/flutter_audio_capture.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'audio_capture_bloc.freezed.dart';

@freezed
abstract class AudioCaptureEvent with _$AudioCaptureEvent {
  const factory AudioCaptureEvent.newData(Float64List buffer) = NewData;
  const factory AudioCaptureEvent.startListening() = StartListening;
  const factory AudioCaptureEvent.stopListening() = StopListening;
}

@freezed
abstract class AudioCaptureState with _$AudioCaptureState {
  const factory AudioCaptureState.initial() = Initial;
  const factory AudioCaptureState.buffer(Float64List buffer) = Buffer;
}

class AudioCaptureBloc extends Bloc<AudioCaptureEvent, AudioCaptureState> {
  AudioCaptureBloc() : super(const AudioCaptureState.initial()) {
    on<AudioCaptureEvent>((event, emit) async {
      await event.when(
          newData: (buffer) async => emit(AudioCaptureState.buffer(buffer)),
          startListening: () async => await _startListeningHandler(),
          stopListening: () async => await _stopListeningHandler());
    });
  }

  final FlutterAudioCapture _audioCapture = FlutterAudioCapture();

  Future<void> _startListeningHandler() async {
    await _audioCapture.start((dynamic obj) {
      var buffer = Float64List.fromList(obj.cast<double>());
      add(AudioCaptureEvent.newData(buffer));
    }, onError, sampleRate: 16000, bufferSize: 3000);
  }

  Future<void> _stopListeningHandler() async {
    await _audioCapture.stop();
  }
}
