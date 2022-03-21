import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sound_test/api.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:sound_test/models/settings_model.dart';

part 'tuner_bloc.freezed.dart';

@freezed
abstract class TunerState with _$TunerState {
  const factory TunerState.initialPitch(NoteResult noteResult) = InitialPitch;
  const factory TunerState.pitchDetected(NoteResult noteResult) = PitchDetected;
  const factory TunerState.noPitchDetected() = NoPitchDetected;
  const factory TunerState.algorithmChanged(DetectionAlgorithm algorithm) =
      AlgorithmChanged;
  const factory TunerState.error(String error) = Error;
}

@freezed
abstract class TunerEvent with _$TunerEvent {
  const factory TunerEvent.startup(int sampleRate, int bufferSize) = _Startup;
  const factory TunerEvent.bufferReady(Float64List buffer) = _BufferReady;
  const factory TunerEvent.changeAlgorithm(DetectionAlgorithm algorithm) =
      _ChangeAlgorithm;
}

class TunerBloc extends Bloc<TunerEvent, TunerState> {
  TunerBloc()
      : super(TunerState.initialPitch(NoteResult(
          noteName: 'A',
          octave: 4,
          centsOffset: 0.0,
          inTune: true,
          previousNoteName: 'G#',
          nextNoteName: 'A#',
        ))) {
    on<TunerEvent>((event, emit) async {
      await event.when(
          startup: (sampleRate, bufferSize) async =>
              await _handleStartup(sampleRate, bufferSize, emit),
          changeAlgorithm: (algorithm) async =>
              await _handleChangeAlgorithm(algorithm, emit),
          bufferReady: (buffer) async =>
              await _handleBufferReady(buffer, emit));
    });
  }
  TunerRs? _tunerApi;

  Future<void> _handleChangeAlgorithm(
      DetectionAlgorithm algorithm, Emitter<TunerState> emit) async {
    await _tunerApi!.changeAlgorithm(algorithm: algorithm.toShortString());
    emit(TunerState.algorithmChanged(algorithm));
  }

  Future<void> _handleStartup(
      int sampleRate, int bufferSize, Emitter<TunerState> emit) async {
    WidgetsFlutterBinding.ensureInitialized();
    final prefs = await SharedPreferences.getInstance();
    final algorithmIdx = prefs.getInt(kSharedPreferencesAlgorithmKey) ??
        SettingsModel.defaultAlgorithm.index;
    _tunerApi = TunerRs(Platform.isAndroid
        ? DynamicLibrary.open('libtuner_rs.so')
        : DynamicLibrary.process());

    final algorithm = DetectionAlgorithm.values[algorithmIdx];
    await _tunerApi!.initTuner(
        algorithm: algorithm.toShortString(),
        sampleRate: sampleRate.toDouble(),
        numSamples: bufferSize);
    emit(TunerState.algorithmChanged(algorithm));
  }

  Future<void> _handleBufferReady(
      Float64List buffer, Emitter<TunerState> emit) async {
    try {
      final pitch = await _tunerApi!.detectPitchWithBuffer(buffer: buffer);

      // I'm not sure why pitch sometimes has a centsOffset of NAN
      if (pitch == null) {
        emit(const TunerState.noPitchDetected());
      } else {
        emit(TunerState.pitchDetected(pitch));
      }
    } catch (e) {
      emit(TunerState.error(e.toString()));
    }
  }
}
