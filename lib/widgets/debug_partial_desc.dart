import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';

class DebugPartialDesc extends StatelessWidget {
  const DebugPartialDesc(this._partialsModel, {Key? key}) : super(key: key);

  final PartialsModel _partialsModel;

  @override
  Widget build(BuildContext context) {
    // final partial = Provider.of<PartialsModel>(context);
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Card(
        color: Theme.of(context).colorScheme.surface,
        child: ListTile(
          leading: Icon(Icons.music_note,
              color: Theme.of(context).colorScheme.onSurface),
          title: Text(
            'Frequency: ${_partialsModel.freq.toStringAsFixed(2)}',
            style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
          ),
          subtitle: Text(
              'Intensity: ${_partialsModel.intensity.toStringAsFixed(2)}',
              style: TextStyle(color: Theme.of(context).colorScheme.onSurface)),
        ),
      ),
    );
  }
}
