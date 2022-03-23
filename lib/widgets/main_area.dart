import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:sound_test/widgets/played_pitch.dart';
import 'package:sound_test/widgets/cents_ruler.dart';
import 'package:sound_test/widgets/pitch_pointer.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';

class MainArea extends StatelessWidget {
  const MainArea({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocConsumer<TunerBloc, TunerState>(
        listener: (context, state) {
          if (state is NoPitchDetected) {
            const snackBar = SnackBar(
              content: Text(
                  'No pitch detected. Try using a different detection algorithm.'),
            );
            ScaffoldMessenger.of(context).removeCurrentSnackBar();
            ScaffoldMessenger.of(context).showSnackBar(snackBar);
          } else if (state is PitchDetected) {
            ScaffoldMessenger.of(context).removeCurrentSnackBar();
          }
        },
        buildWhen: (_, current) => current is PitchDetected,
        builder: (context, state) {
          return state.maybeWhen(
              pitchDetected: (pitch) {
                return Column(
                  children: [
                    Expanded(flex: 5, child: PlayedPitch(pitch)),
                    const SizedBox(height: 20),
                    const Expanded(
                      flex: 1,
                      child: CentsRuler(),
                    ),
                    Expanded(
                      flex: 2,
                      child: PitchPointer(pitch),
                    ),
                    const SizedBox(height: 40),
                  ],
                );
              },
              error: (errorString) =>
                  Text('Stack trace: ${errorString.toString()}'),
              orElse: () => const Center(child: CircularProgressIndicator()));
        });
  }
}
