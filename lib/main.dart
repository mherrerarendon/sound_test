import 'package:flutter/material.dart';
import 'package:sound_test/widgets/main_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/fft_peak.dart';

void main() {
  runApp(ChangeNotifierProvider(
    create: (context) => FftPeakModel(),
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
        primarySwatch: Colors.blue,
      ),
      home: const MainWidget(),
    );
  }
}
