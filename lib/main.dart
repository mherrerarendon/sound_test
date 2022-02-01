import 'package:flutter/material.dart';
import 'package:sound_test/widgets/main_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final prefs = await SharedPreferences.getInstance();
  final algorithmIdx = prefs.getInt(kSharedPreferencesAlgorithmKey) ??
      SettingsModel.defaultAlgorithm.index;
  runApp(MultiProvider(providers: [
    ChangeNotifierProvider(
      create: (_) => PartialsModel(),
    ),
    ChangeNotifierProvider(
      create: (_) => SettingsModel(algorithmIdx),
    ),
  ], child: TunerInherited(child: const MyApp())));
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
