import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/algorithm_button.dart';

class AppDrawer extends StatelessWidget {
  const AppDrawer({Key? key}) : super(key: key);

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
          child: AlgorithmButton(),
        ),
      ],
    ));
  }
}
