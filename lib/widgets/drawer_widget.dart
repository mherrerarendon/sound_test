import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/select_algorithm_widget.dart';

class AppDrawer extends StatelessWidget {
  const AppDrawer({Key? key}) : super(key: key);

  void _pushSelectAlgorithm(BuildContext context) {
    Navigator.of(context)
        .push(MaterialPageRoute(builder: (BuildContext context) {
      return const SelectAlgorithmPage();
    }));
  }

  @override
  Widget build(BuildContext context) {
    return Drawer(
        child: ListView(
      padding: EdgeInsets.zero,
      children: <Widget>[
        DrawerHeader(
          decoration: const BoxDecoration(
            color: Colors.blue,
          ),
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxHeight: 40, maxWidth: 200),
              child: OutlinedButton(
                child: Consumer<SettingsModel>(
                    builder: (context, settings, _) => Text(
                          settings.detectionAlgorithm.toName(),
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 24,
                          ),
                        )),
                onPressed: () {
                  _pushSelectAlgorithm(context);
                },
              ),
            ),
          ),
        ),
      ],
    ));
  }
}
