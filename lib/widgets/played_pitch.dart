import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

const double kWidth = 40;

class PlayedPitch extends StatelessWidget {
  const PlayedPitch({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(
      builder: (context, partials, _) {
        return Row(
          children: [
            SizedBox(
              width: kWidth,
              child: FittedBox(
                child: Text(
                  partials.leftNoteName,
                  style: TextStyle(
                      color: partials.inTune()
                          ? Theme.of(context).colorScheme.onSurface
                          : Colors.grey),
                ),
              ),
            ),
            const SizedBox(
              width: 10,
            ),
            Expanded(
              child: FittedBox(
                alignment: Alignment.bottomCenter,
                child: Text(
                  partials.noteName,
                  style: TextStyle(
                      color: partials.inTune()
                          ? Theme.of(context).colorScheme.onSurface
                          : Colors.grey),
                ),
              ),
            ),
            const SizedBox(
              width: 10,
            ),
            SizedBox(
              width: kWidth,
              child: FittedBox(
                child: Text(
                  partials.rigthNoteName,
                  style: TextStyle(
                      color: partials.inTune()
                          ? Theme.of(context).colorScheme.onSurface
                          : Colors.grey),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
