import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/widgets/partial_desc.dart';
import 'package:sound_test/api.dart';

class AllPartials extends StatelessWidget {
  const AllPartials({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(builder: (context, partials, _) {
      return Center(
        child: ListView.builder(
          scrollDirection: Axis.vertical,
          shrinkWrap: true,
          itemCount: partials.numPartials,
          itemBuilder: (context, index) {
            final Partial partial = partials.getPartial(index);
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
