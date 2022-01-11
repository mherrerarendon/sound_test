import 'package:flutter/material.dart';
import 'package:sound_test/widgets/partial_desc.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/pitch_slider.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:sound_test/widgets/all_partials.dart';
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
          // Consumer<PartialsModel>(builder: (context, fftPeakModel, _) {
          //   return PartialDesc(fftPeakModel.freq.toStringAsFixed(2),
          //       fftPeakModel.intensity.toStringAsFixed(2));
          // }),
          AllPartials(),
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
