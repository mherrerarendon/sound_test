import 'dart:typed_data';

import 'package:flutter_audio_capture/flutter_audio_capture.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:wakelock/wakelock.dart';

part 'audio_capture_bloc.freezed.dart';

const int kSampleRate = 44100;
const int kBufferSize = 22050;

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
  const factory AudioCaptureState.listening() = Listening;
  const factory AudioCaptureState.notListening() = NotListening;
}

class AudioCaptureBloc extends Bloc<AudioCaptureEvent, AudioCaptureState> {
  AudioCaptureBloc() : super(const AudioCaptureState.initial()) {
    on<AudioCaptureEvent>((event, emit) async {
      await event.when(
          newData: (buffer) async => emit(AudioCaptureState.buffer(buffer)),
          startListening: () async => await _startListeningHandler(emit),
          stopListening: () async => await _stopListeningHandler(emit));
    });
  }

  final FlutterAudioCapture _audioCapture = FlutterAudioCapture();

  Future<void> _startListeningHandler(Emitter<AudioCaptureState> emit) async {
    if (state is! Listening) {
      await _audioCapture.start((dynamic obj) {
        var buffer = Float64List.fromList(obj.cast<double>());
        add(AudioCaptureEvent.newData(buffer));
      }, onError, sampleRate: kSampleRate, bufferSize: kBufferSize);
      emit(const AudioCaptureState.listening());
    }
    Wakelock.enable();
  }

  Future<void> _stopListeningHandler(Emitter<AudioCaptureState> emit) async {
    if (state is! NotListening) {
      await _audioCapture.stop();
      emit(const AudioCaptureState.notListening());
    }
    Wakelock.disable();
  }
}
