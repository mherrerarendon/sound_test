import 'dart:ffi';
import 'dart:io';
import 'dart:async';
import 'dart:math';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:sound_test/api.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:dartz/dartz.dart';
import 'package:sound_test/models/settings_model.dart';

part 'tuner_bloc.freezed.dart';

// Use union class instead (for failures)
class TunerState {
  late final Option<Pitch> pitch;
  TunerState._(this.pitch);

  factory TunerState({
    @required Pitch? pitch,
  }) {
    if (pitch == null) {
      return TunerState._(
        none(),
      );
    }
    return TunerState._(some(pitch));
  }

  factory TunerState.initial() => TunerState._(none());
}

@freezed
abstract class TunerEvent with _$TunerEvent {
  const factory TunerEvent.startup(DetectionAlgorithm algorithm) = _Startup;
  const factory TunerEvent.bufferReady(Uint8List buffer) = _BufferReady;
}

class TunerBloc extends Bloc<TunerEvent, TunerState> {
  TunerBloc() : super(TunerState.initial()) {
    on<TunerEvent>((event, emit) {
      event.when(
          startup: _handleStartup,
          bufferReady: (buffer) => _handleBufferReady(buffer, emit));
    });
  }
  TunerRs? _tunerApi;

  void _handleStartup(DetectionAlgorithm algorithm) async {
    _tunerApi = TunerRs(Platform.isAndroid
        ? DynamicLibrary.open('libtuner_rs.so')
        : DynamicLibrary.process());
    await _tunerApi!.initTuner(algorithm: algorithm.toShortString());
  }

  void _handleBufferReady(Uint8List buffer, Emitter<TunerState> emit) async {
    try {
      final pitch = await _tunerApi!.detectPitchWithBuffer(byteBuffer: buffer);
      emit()
      _controller.add(pitch);
    } catch (e) {
      _controller.addError(e);
    }
    emit(TunerState(pitch: pitch.pitch, confidence: pitch.confidence));
  }
}
