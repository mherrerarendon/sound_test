import 'package:flutter/material.dart';
import 'package:sound_test/widgets/label_box.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/pitch_slider.dart';
import 'package:sound_test/models/fft_peak.dart';
import 'package:provider/provider.dart';
// import 'package:permission_handler/permission_handler.dart';

class MainWidget extends StatefulWidget {
  const MainWidget({Key? key}) : super(key: key);

  @override
  _MainWidgetState createState() => _MainWidgetState();
}

class _MainWidgetState extends State<MainWidget> {
  // ----------------------------------------------------------------------------------------------------------------------

  @override
  Widget build(BuildContext context) {
    Widget makeBody() {
      return Column(
        children: [
          const ListenWidget(),
          PitchSlider(),
          Consumer<FftPeakModel>(builder: (context, fftPeakModel, _) {
            return LabelBox('Frequency', fftPeakModel.freq.toStringAsFixed(2));
          }),
          Consumer<FftPeakModel>(builder: (context, fftPeakModel, _) {
            return LabelBox(
                'Intensity', fftPeakModel.intensity.toStringAsFixed(2));
          }),
        ],
      );
    }

    return Scaffold(
      backgroundColor: Colors.blue,
      appBar: AppBar(
        title: const Text('Sound Test'),
      ),
      body: makeBody(),
    );
  }
}
