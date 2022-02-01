import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';
import 'package:sound_test/widgets/algorithm_details.dart';

class SelectAlgorithmPage extends StatelessWidget {
  const SelectAlgorithmPage({Key? key}) : super(key: key);

  Column buildListView(SettingsModel settings, BuildContext context) {
    final radioList = DetectionAlgorithm.values.map((algorithm) {
      return RadioListTile<DetectionAlgorithm>(
        title: Text(algorithm.toName(), style: TextStyle(fontSize: 24)),
        groupValue: settings.detectionAlgorithm,
        value: algorithm,
        onChanged: (DetectionAlgorithm? val) async {
          settings.setDetectionAlgorithm(val!);
          await TunerInherited.of(context)!
              .tunerApi
              .setAlgorithm(algorithm: algorithm.toShortString());
          await Future.delayed(const Duration(milliseconds: 350));
          Navigator.pop(context);
        },
      );
    }).toList();
    return Column(
      children: radioList,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
          title: Text(
            'Pitch Detection Algorithm',
            style: TextStyle(fontSize: 24),
          ),
          actions: <Widget>[
            IconButton(
              icon: const Icon(Icons.help),
              tooltip: 'Help me choose',
              onPressed: () {
                final settings =
                    Provider.of<SettingsModel>(context, listen: false);
                Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (context) =>
                          AlgorithmDetails(settings.detectionAlgorithm.index)),
                );
              },
            ),
          ]),
      body: Consumer<SettingsModel>(
          builder: (context, settings, _) => buildListView(settings, context)),
    );
  }
}
