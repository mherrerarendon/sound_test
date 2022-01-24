import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

class PlayedPitch extends StatelessWidget {
  const PlayedPitch({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
        final double height = constraints.maxHeight;
        return SizedBox(
            height: height,
            child: Center(
              child: Consumer<PartialsModel>(
                builder: (context, partials, _) {
                  final centsOffset = partials.centsOffset;
                  return Column(
                    children: [
                      Text(
                          'Cents offset: ${centsOffset > 0 ? '+' : ''}${centsOffset.toStringAsFixed(2)}',
                          style: TextStyle(
                              fontSize: 20,
                              color: partials.inTune()
                                  ? Theme.of(context).colorScheme.onSurface
                                  : centsOffset < 0
                                      ? Colors.red
                                      : Colors.blue)),
                      Text(
                        partials.noteName,
                        style: TextStyle(
                            fontSize: height *
                                .7, // .9 to make it look nice and not too tight
                            color: partials.inTune()
                                ? Theme.of(context).colorScheme.onSurface
                                : Colors.grey),
                      ),
                    ],
                  );
                },
              ),
            ));
      }),
    );
  }
}
