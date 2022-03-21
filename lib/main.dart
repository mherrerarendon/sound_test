import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:sound_test/blocs/audio_capture_bloc.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:sound_test/widgets/main_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final prefs = await SharedPreferences.getInstance();
  final algorithmIdx = prefs.getInt(kSharedPreferencesAlgorithmKey) ??
      SettingsModel.defaultAlgorithm.index;
  runApp(MultiProvider(
      providers: [
        ChangeNotifierProvider(
          create: (_) => SettingsModel(algorithmIdx),
        ),
      ],
      child: BlocProvider(
        create: (context) => TunerBloc()
          ..add(const TunerEvent.startup(kSampleRate, kBufferSize)),
        child: const MyApp(),
      )));
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
