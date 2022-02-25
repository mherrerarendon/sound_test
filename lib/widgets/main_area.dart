import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';
import 'package:sound_test/widgets/played_pitch.dart';
import 'package:sound_test/widgets/cents_ruler.dart';
import 'package:sound_test/widgets/pitch_pointer.dart';
import 'package:sound_test/widgets/debug_partial_desc.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/constants.dart';

const bool _debug = false;

class MainArea extends StatelessWidget {
  const MainArea({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final tuner = TunerInherited.of(context)!;
    return StreamBuilder<Partial>(
        // stream: _bids,
        stream: tuner.stream,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text('Stack trace: ${snapshot.stackTrace}');
          } else {
            switch (snapshot.connectionState) {
              case ConnectionState.none:
                return const Text('Connection State: none');
              case ConnectionState.waiting:
                return const Text('Connection State: waiting');
              case ConnectionState.active:
                final partialModel = PartialsModel(snapshot.data!);
                if (partialModel.freq < tMinFrequency) {
                  return const Text('Connection State: frequency too low');
                }
                return Column(
                  children: [
                    _debug ? DebugPartialDesc(partialModel) : Container(),
                    Expanded(flex: 5, child: PlayedPitch(partialModel)),
                    const SizedBox(height: 20),
                    const Expanded(
                      flex: 1,
                      child: CentsRuler(),
                    ),
                    Expanded(
                      flex: 2,
                      child: PitchPointer(partialModel),
                    ),
                    const SizedBox(height: 40),
                  ],
                );
              case ConnectionState.done:
                return Text('\$${snapshot.data} (closed)');
            }
          }
        });
  }
}
