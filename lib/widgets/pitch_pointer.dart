import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

class PitchPointer extends StatelessWidget {
  const PitchPointer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(
      builder: (context, partials, _) {
        final centsOffset = partials.centsOffset;
        return Text(
            'Cents offset: ${centsOffset > 0 ? '+' : ''}${centsOffset.toStringAsFixed(2)}',
            style: TextStyle(
                fontSize: 20,
                color: partials.inTune()
                    ? Theme.of(context).colorScheme.onSurface
                    : centsOffset < 0
                        ? Colors.red
                        : Colors.blue));
      },
    );
  }
}
