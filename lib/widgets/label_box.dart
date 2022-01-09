import 'package:flutter/material.dart';

class LabelBox extends StatelessWidget {
  final String label;
  final String value;
  const LabelBox(this.label, this.value, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(3),
      padding: const EdgeInsets.all(3),
      height: 80,
      width: double.infinity,
      alignment: Alignment.centerLeft,
      decoration: BoxDecoration(
        color: const Color(0xFFFAF0E6),
        border: Border.all(
          color: Colors.indigo,
          width: 3,
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Text('$label: $value'),
      ),
    );
  }
}
