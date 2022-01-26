import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/widgets/partial_desc.dart';

class AllPartials extends StatelessWidget {
  const AllPartials({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(builder: (context, partial, _) {
      return Center(
        child: ListView.builder(
          scrollDirection: Axis.vertical,
          shrinkWrap: true,
          itemCount: 1,
          itemBuilder: (context, index) {
            return SizedBox(
              child: PartialDesc(partial.freq.toStringAsFixed(2),
                  partial.intensity.toStringAsFixed(2)),
            );
          },
        ),
      );
    });
  }
}
