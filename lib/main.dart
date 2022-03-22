import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:sound_test/blocs/audio_capture_bloc.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:sound_test/widgets/main_widget.dart';

void main() async {
  runApp(BlocProvider(
    create: (context) => TunerBloc()
      ..add(TunerEvent.startup(kSampleRate,
          Platform.isAndroid ? kAndroidBufferSize : kIosBufferSize)),
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
