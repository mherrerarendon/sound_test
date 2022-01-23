import 'package:flutter/material.dart';
import 'package:sound_test/widgets/select_algorithm_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';

class AlgorithmButton extends StatelessWidget {
  const AlgorithmButton({Key? key}) : super(key: key);

  void _pushSelectAlgorithm(BuildContext context) {
    Navigator.of(context)
        .push(MaterialPageRoute(builder: (BuildContext context) {
      return const SelectAlgorithmPage();
    }));
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxHeight: 60 /*, maxWidth: 200*/),
        child: OutlinedButton(
          child: Consumer<SettingsModel>(
              builder: (context, settings, _) => Text(
                    settings.detectionAlgorithm.toName(),
                    style: const TextStyle(
                      // color: Colors.white,
                      fontSize: 40,
                    ),
                  )),
          onPressed: () {
            _pushSelectAlgorithm(context);
          },
          style: TextButton.styleFrom(
            primary: Colors.white,
            // primary: Theme.of(context).colorScheme.onSurface,
            onSurface: Colors.black,
            // minimumSize: Size(88, 36),
            // padding: EdgeInsets.symmetric(horizontal: 16.0),
            shape: const RoundedRectangleBorder(
              borderRadius: BorderRadius.all(Radius.circular(2.0)),
            ),
          ),
        ),
      ),
    );
  }
}
