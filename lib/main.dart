import 'package:flutter/material.dart';
import 'package:sound_test/widgets/main_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

void main() {
  runApp(ChangeNotifierProvider(
    create: (context) => PartialsModel(),
    child: const MyApp(),
  ));
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        brightness: Brightness.light,
        /* light theme settings */
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        /* dark theme settings */
      ),
      themeMode: ThemeMode.system,
      home: const MainWidget(),
    );
  }
}
