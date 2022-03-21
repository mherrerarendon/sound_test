import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/blocs/audio_capture_bloc.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';

class ListenWidget extends StatefulWidget {
  const ListenWidget({
    Key? key,
  }) : super(key: key);

  @override
  _ListenWidgetState createState() => _ListenWidgetState();
}

class _ListenWidgetState extends State<ListenWidget> {
  @override
  void dispose() {
    context
        .read<AudioCaptureBloc>()
        .add(const AudioCaptureEvent.stopListening());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) =>
          AudioCaptureBloc()..add(const AudioCaptureEvent.startListening()),
      child: BlocConsumer<AudioCaptureBloc, AudioCaptureState>(
        listener: (context, state) {
          state.maybeWhen(
              buffer: (buffer) =>
                  context.read<TunerBloc>().add(TunerEvent.bufferReady(buffer)),
              orElse: () {});
        },
        buildWhen: (_, current) =>
            current is Listening || current is NotListening,
        builder: (context, state) {
          return FloatingActionButton(
            onPressed: () {
              if (state is Listening) {
                context
                    .read<AudioCaptureBloc>()
                    .add(const AudioCaptureEvent.stopListening());
              } else if (state is NotListening) {
                context
                    .read<AudioCaptureBloc>()
                    .add(const AudioCaptureEvent.startListening());
              }
            },
            tooltip: state is Listening ? 'Listening...' : 'Not listening...',
            child: Icon(
                state is Listening ? Icons.mic_outlined : Icons.mic_off_sharp),
          );
        },
      ),
    );
  }
}
